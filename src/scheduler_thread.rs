use super::job::*;

use std::sync::*;
use std::thread::*;
use std::sync::mpsc::*;

///
/// A scheduler thread reads from the scheduler queue
///
pub struct SchedulerThread {
    /// The jobs that this thread should run
    jobs: Sender<Box<ScheduledJob>>,

    /// The thread itself
    pub thread: JoinHandle<()>,

    /// Flag that indicates that this thread is busy
    pub busy: Arc<Mutex<bool>>
}

impl SchedulerThread {
    ///
    /// Creates a new scheduler thread 
    ///
    pub fn new() -> SchedulerThread {
        // All the thread does is run jobs from its channel
        let (jobs_in, jobs_out): (Sender<Box<ScheduledJob>>, Receiver<Box<ScheduledJob>>) = channel();
        let thread = spawn(move || {
            while let Ok(mut job) = jobs_out.recv() {
                job.run();
            }
        });

        SchedulerThread {
            jobs:   jobs_in,
            thread: thread,
            busy:   Arc::new(Mutex::new(false))
        }
    }

    ///
    /// Schedules a job to be run on this thread
    ///
    pub fn run<Job: 'static+ScheduledJob>(&self, job: Job) {
        self.jobs.send(Box::new(job)).unwrap();
    }
}
