#[allow(unused_macros)]
#[macro_export]
macro_rules! on_match_all {
    () => {
        fn match_(&self, _: &mut MessageEvent) -> bool {
            true
        }
    };
}

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
    ($event_type: ty, $($x:expr),*) => {
        fn match_(&self, event: &mut $event_type) -> bool {
            let mut commands:Vec<&str> = Vec::new();
            $(
                commands.push($x);
            )*
            for command in commands.iter() {
                if event.get_raw_message().starts_with(command) {
                    event.set_raw_message(event.get_raw_message().replace(command, "").to_string());
                    return true;
                }
            }
            false
        }
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! on_start_with {
    ($event_type: ty, $command: expr) => {
        fn match_(&self, event: &mut $event_type) -> bool {
            if event.get_raw_message().starts_with($command) {
                true
            } else {
                false
            }
        }
    };
    ($event_type: ty, $($x:expr),*) => {
        fn match_(&self, event: &mut $event_type) -> bool {
            let mut commands:Vec<&str> = Vec::new();
            $(
                commands.push($x);
            )*
            for command in commands.iter() {
                if event.get_raw_message().starts_with(command) {
                    return true;
                }
            }
            false
        }
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! matcher_request {
    ($b:block) => {
        #[derive(Clone)]
        struct Temp {}

        #[async_trait]
        impl Handler<MessageEvent> for Temp {
            on_match_all!();
            async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                $b
            }
        }

        matcher
            .set_message_matcher(
                event.get_self_id(),
                build_temp_message_event_matcher(&event, Temp {}),
            )
            .await;
    }; // #[derive(Clone)]
       // struct Temp {}

       // #[async_trait]
       // impl Handler<MessageEvent> for Temp {
       //     on_match_all!();
       //     async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
       //         let msg = event.get_raw_message();
       //         matcher.send_text(&encode(&msg)).await;
       //     }
       // }

       // matcher
       //     .set_message_matcher(
       //         event.get_self_id(),
       //         build_temp_message_event_matcher(&event, Temp {}),
       //     )
       //     .await;
}
