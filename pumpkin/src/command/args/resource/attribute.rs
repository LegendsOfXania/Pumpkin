use pumpkin_data::attributes::Attributes;
use pumpkin_protocol::java::client::play::{ArgumentType, SuggestionProviders};
use crate::command::args::{Arg, ArgumentConsumer, ConsumeResult, ConsumedArgs, DefaultNameArgConsumer, FindArg, GetClientSideArgParser};
use crate::command::CommandSender;
use crate::command::dispatcher::CommandError;
use crate::command::tree::RawArgs;
use crate::server::Server;

pub struct AttributeArgumentConsumer;

impl GetClientSideArgParser for AttributeArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType<'_> {
        ArgumentType::Resource {
            identifier: "attribute",
        }
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

impl ArgumentConsumer for AttributeArgumentConsumer {
    fn consume<'a, 'b>(
        &'a self,
        _sender: &'a CommandSender,
        _server: &'a Server,
        args: &'b mut RawArgs<'a>,
    ) -> ConsumeResult<'a> {
        let name_opt: Option<&'a str> = args.pop();

        let result: Option<Arg<'a>> = name_opt.map_or_else(
            || None,
            |name| {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                Attributes::from_name(name).map(Arg::Attribute)
            },
        );

        Box::pin(async move { result })
    }
}

impl DefaultNameArgConsumer for AttributeArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "attribute"
    }
}


impl<'a> FindArg<'a> for AttributeArgumentConsumer {
    type Data = &'a Attributes;

    fn find_arg(args: &'a ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Attribute(data)) => Ok(data),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}