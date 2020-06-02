use {
    std::{

        pin::Pin,
        task::{Context, Poll},
        thread,
      //  time::Duration,
    },
};
extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use {
    futures::{
        future::{Future},

    },

};


#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}

impl WaitForIt {
    pub fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            polls: 0,
            message: message,
            until: Utc::now() + delay,
        }
    }
}

impl Future for WaitForIt {
    type Output = String;


    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Utc::now();
        thread::sleep(::std::time::Duration::from_millis(500));
        if self.until < now {
            Poll::Ready(
                format!("{} after {} polls!", self.message, self.polls)
            )
        } else {
            self.polls += 1;

            println!("not ready yet --> {:?}", self);
            if self.message.eq(&String::from("I'm done:")) {
                _cx.waker().clone().wake();
            }
            Poll::Pending
        }
    }
}


#[test]
fn wait_for_test() {
    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    async_std::task::block_on(async {let res =  wfi_1.await; println!("out:{}",res)});
}