#[allow(unused_macros)]
#[macro_export]
macro_rules! on_command {
    ($event_type: ty, $command: expr) => {
        fn match_(&self, event: &mut $event_type) -> bool {
            if event.get_raw_message().starts_with($command) {
                event.set_raw_message(event.get_raw_message().replace($command, "").to_string());
                true
            } else {
                false
            }
        }
    }; // fn match_(&self, event: &mut MessageEvent) -> bool {
       //     if event.get_raw_message().starts_with(r"echo ") {
       //         event.set_raw_message(event.get_raw_message().replace(r"echo ", "").to_string());
       //         true
       //     } else {
       //         false
       //     }
       // }
}
