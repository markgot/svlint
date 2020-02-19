use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

pub struct EnumWithType;

impl Rule for EnumWithType {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::DataTypeEnum(x) => {
                let (_, ref a, _, _) = x.nodes;
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("enum_with_type")
    }

    fn hint(&self) -> String {
        String::from("`enum` must have data type")
    }

    fn reason(&self) -> String {
        String::from("the default data type is `int`")
    }
}
