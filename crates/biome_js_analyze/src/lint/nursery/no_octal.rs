use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsNumberLiteralExpression;
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow octal literals
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var num = 071;
    /// var result = 5 + 07;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var num = 71;
    /// var result = 5 + 7;
    /// var num = "071";
    /// ```
    ///
    pub NoOctal {
        version: "1.0.0",
        name: "noOctal",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoOctal {
    type Query = Semantic<JsNumberLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let number_literal_expression = ctx.query();
        let _model = ctx.model();
        let value = number_literal_expression.value_token().ok()?;
        let is_octal = value.text().starts_with('0');

        println!("{:?}", is_octal);

        return None;

        // let object = member_expression.object().ok()?;
        // let (reference, name) = global_identifier(&object)?;
        // if name.text() != "console" {
        //     return None;
        // }
        // model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let node = JsNumberLiteralExpression::cast(node.syntax().parent()?)?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Don't use octal literals"
                },
            )
            .note(markup! {
                <Emphasis>"console.log"</Emphasis>" is usually a tool for debugging and you don't want to have that in production."
            })
            .note(markup! {
                "If it is not for debugging purpose then using "<Emphasis>"console.info"</Emphasis>" might be more appropriate."
            }),
        )
    }
}
