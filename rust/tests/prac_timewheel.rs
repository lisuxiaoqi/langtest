use std::time::Duration;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct TimerTask {
    round: usize,
    task: Task,
}

struct TimeWheel {
    slots: Vec<Vec<TimerTask>>,
    cur_slot: usize,
    cap: usize,
}

impl TimeWheel {
    fn new(cap: usize) -> Self {
        let mut slots = Vec::<Vec<TimerTask>>::with_capacity(cap);
        for _ in 0..cap {
            slots.push(Vec::new());
        }

        Self {
            slots,
            cur_slot: 0,
            cap,
        }
    }

    fn spawn(&mut self, dur: usize, task: impl FnOnce() + Send + 'static) {
        let slot = (dur + self.cur_slot) % self.cap;
        let timer_task = TimerTask {
            round: dur / self.cap,
            task: Box::new(task),
        };
        self.slots[slot].push(timer_task);
    }

    fn tick(&mut self) {
        println!("tick slot {}", self.cur_slot);
        let mut remain_tasks = Vec::new();

        for mut timer_task in self.slots[self.cur_slot].drain(..) {
            if timer_task.round > 0 {
                timer_task.round -= 1;
                remain_tasks.push(timer_task);
            } else {
                (timer_task.task)();
            }
        }

        self.slots[self.cur_slot] = remain_tasks;
        self.cur_slot = (self.cur_slot + 1) % self.cap;
    }

    fn start(&mut self) {
        for _ in 0..20 {
            self.tick();
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

#[test]
fn test_timewheel() {
    let mut wheel = TimeWheel::new(8);

    wheel.spawn(3, || {
        println!("task after 3 seconds");
    });

    wheel.spawn(10, || {
        println!("task after 10 seconds");
    });

    wheel.start();
}
