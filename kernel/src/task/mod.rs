//! Sodda kooperativ scheduler — Faza 4.
//!
//! Bu minimal versiya: preemptive emas, kontekst almashtirish yo'q.
//! Har bir task — `FnMut` body, `iteration` argument bilan chaqiriladi.
//! Body `true` qaytarsa, task davom etadi; `false` qaytarsa — tugaydi.
//! Round-robin tartibida ishlaydi.
//!
//! Faza 7+ da preemptive (timer IRQ asosida) va alohida stack/registr
//! kontekst almashtirish qo'shiladi.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::string::String;
use core::cell::RefCell;

pub type TaskBody = Box<dyn FnMut(u32) -> bool>;

struct Task {
    name: String,
    iteration: u32,
    body: TaskBody,
}

struct Scheduler {
    queue: VecDeque<Task>,
}

impl Scheduler {
    const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn spawn(&mut self, name: &str, body: TaskBody) {
        self.queue.push_back(Task {
            name: String::from(name),
            iteration: 0,
            body,
        });
    }

    fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            task.iteration += 1;
            let alive = (task.body)(task.iteration);
            if alive {
                self.queue.push_back(task);
            } else {
                crate::println!("[sched] task '{}' tugadi", task.name);
            }
        }
        crate::println!("[sched] hamma task tugadi.");
    }
}

struct GlobalScheduler(RefCell<Scheduler>);
unsafe impl Sync for GlobalScheduler {}

static SCHED: GlobalScheduler = GlobalScheduler(RefCell::new(Scheduler::new()));

pub fn spawn<F>(name: &str, body: F)
where
    F: FnMut(u32) -> bool + 'static,
{
    SCHED.0.borrow_mut().spawn(name, Box::new(body));
}

pub fn run() {
    SCHED.0.borrow_mut().run();
}
