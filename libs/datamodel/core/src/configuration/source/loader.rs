use super::{
    builtin::{MySqlSourceDefinition, PostgresSourceDefinition, SqliteSourceDefinition},
    traits::{Source, SourceDefinition},
};
use crate::ast;
use crate::common::arguments::Arguments;
use crate::common::value_validator::ValueListValidator;
use crate::error::{DatamodelError, ErrorCollection};
use crate::StringFromEnvVar;
use datamodel_connector::{Connector, ExampleConnector, MultiProviderConnector};

/// Helper struct to load and validate source configuration blocks.
pub struct SourceLoader {
    source_definitions: Vec<Box<dyn SourceDefinition>>,
}

impl SourceLoader {
    /// Creates a new, empty source loader.
    pub fn new() -> Self {
        Self {
            source_definitions: get_builtin_sources(),
        }
    }

    /// Loads all source config blocks form the given AST,
    /// and returns a Source instance for each.
    pub fn load_sources(
        &self,
        ast_schema: &ast::SchemaAst,
        ignore_env_var_errors: bool,
    ) -> Result<Vec<Box<dyn Source>>, ErrorCollection> {
        let mut sources = vec![];
        let mut errors = ErrorCollection::new();

        for src in &ast_schema.sources() {
            match self.load_source(&src, ignore_env_var_errors) {
                Ok(loaded_src) => sources.push(loaded_src),
                // Lift error to source.
                Err(DatamodelError::ArgumentNotFound { argument_name, span }) => errors.push(
                    DatamodelError::new_source_argument_not_found_error(&argument_name, &src.name.name, span),
                ),
                Err(err) => errors.push(err),
            }
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(sources)
        }
    }

    /// Internal: Loads a single source from a source config block in the datamodel.
    fn load_source(
        &self,
        ast_source: &ast::SourceConfig,
        ignore_env_var_errors: bool,
    ) -> Result<Box<dyn Source>, DatamodelError> {
        let source_name = &ast_source.name.name;
        let mut args = Arguments::new(&ast_source.properties, ast_source.span);

        let provider_arg = args.arg("provider")?;
        if provider_arg.is_from_env() {
            return Err(DatamodelError::new_functional_evaluation_error(
                &format!("A datasource must not use the env() function in the provider argument."),
                ast_source.span,
            ));
        }
        let providers = provider_arg.as_array().to_str_vec()?;

        if providers.is_empty() {
            return Err(DatamodelError::new_source_validation_error(
                "The provider argument in a datasource must not be empty",
                source_name,
                provider_arg.span(),
            ));
        }

        let url_args = args.arg("url")?;
        let (env_var_for_url, url) = match url_args.as_str_from_env() {
            _ if ignore_env_var_errors => {
                // glorious hack. ask marcus
                (None, format!("{}://", providers.first().unwrap()))
            }
            Ok((env_var, url)) => (env_var, url.trim().to_owned()),
            Err(err) => return Err(err),
        };

        if url.is_empty() {
            let suffix = match &env_var_for_url {
                Some(env_var_name) => format!(
                    " The environment variable `{}` resolved to an empty string.",
                    env_var_name
                ),
                None => "".to_owned(),
            };
            let msg = format!(
                "You must provide a nonempty URL for the datasource `{}`.{}",
                source_name, &suffix
            );
            return Err(DatamodelError::new_source_validation_error(
                &msg,
                source_name,
                url_args.span(),
            ));
        }

        let documentation = ast_source.documentation.clone().map(|comment| comment.text);
        let url = StringFromEnvVar {
            from_env_var: env_var_for_url,
            value: url,
        };

        let source_definitions: Vec<_> = providers
            .iter()
            .filter_map(|provider| self.get_source_definition_for_provider(&provider))
            .collect();

        if source_definitions.is_empty() {
            return Err(DatamodelError::new_datasource_provider_not_known_error(
                &providers.join(","),
                provider_arg.span(),
            ));
        }

        let results: Vec<_> = source_definitions
            .iter()
            .map(|sd| {
                sd.create(
                    source_name,
                    url.clone(),
                    &documentation,
                    self.combined_connector(&providers),
                )
                .map_err(|err_msg| DatamodelError::new_source_validation_error(&err_msg, source_name, url_args.span()))
            })
            .collect();

        // Return the first source that was created successfully.
        // If no source was created successfully return the first of all errors.
        let (successes, errors): (Vec<_>, Vec<_>) = results.into_iter().partition(|result| result.is_ok());
        if !successes.is_empty() {
            Ok(successes.into_iter().next().unwrap()?)
        } else {
            Err(errors.into_iter().next().unwrap().err().unwrap())
        }
    }

    // This is separate function because it is called repetitively in a loop above.
    fn combined_connector(&self, providers: &Vec<String>) -> Box<MultiProviderConnector> {
        let connectors: Vec<Box<dyn Connector>> = providers
            .iter()
            .filter_map(|provider| {
                let connector: Option<Box<dyn Connector>> = match provider.as_str() {
                    "mysql" => Some(Box::new(ExampleConnector::mysql())),
                    "postgres" | "postgresql" => Some(Box::new(ExampleConnector::postgres())),
                    "sqlite" => Some(Box::new(ExampleConnector::sqlite())),
                    _ => None, // if a connector is not known this is handled by the following code
                };
                connector
            })
            .collect();
        Box::new(MultiProviderConnector::new(connectors))
    }

    fn get_source_definition_for_provider(&self, provider: &str) -> Option<&Box<dyn SourceDefinition>> {
        self.source_definitions.iter().find(|sd| sd.is_provider(provider))
    }
}

fn get_builtin_sources() -> Vec<Box<dyn SourceDefinition>> {
    vec![
        Box::new(MySqlSourceDefinition::new()),
        Box::new(PostgresSourceDefinition::new()),
        Box::new(SqliteSourceDefinition::new()),
    ]
}
