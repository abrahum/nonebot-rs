use std::sync::{Arc, Mutex};

// 传入 Events 检查当前当前事件是否需要被响应，true 表示需要
// check the event should be handle or not
type Rule = fn(&crate::event::Events) -> bool;

// #[derive(Clone)]
pub struct Matcher {
    // Matcher 匹配器，每个匹配器对应一个 handle 函数
    rules: Vec<Rule>, // 所有需要被满足的 rule
    block: bool,      // 是否阻止事件向下一级传递
    temp: bool,       // 是否为临时 Matcher
    handler: fn(event: &crate::event::Events, matcher: Matcher, nb: Arc<Mutex<crate::Nonebot>>),
}

impl Matcher {
    pub fn get_rules(&self) -> &Vec<Rule> {
        // 获取当前 Matcher 所有匹配规格
        // get all rules in the Matcher
        &self.rules
    }

    pub fn push_rule(&mut self, rule: Rule) -> Result<(), String> {
        // 给当前 Matcher 增加需要满足的 Rule
        // 可以在此处增加 Rule 的合法性检查
        // check the rule pushable here
        self.rules.push(rule);
        Ok(())
    }

    pub fn check(&self, event: crate::event::Events) -> bool {
        // 一次性检查当前事件是否满足所有 Rule
        // check the event fit all the rules or not
        let mut rbool = true;
        for rule in &self.rules {
            let temp_bool = rule(&event);
            rbool = rbool && temp_bool;
            if !rbool {
                break;
            }
        }
        rbool
    }
}
