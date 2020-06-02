use crate::graph::Graph;
use crate::search::{AStarSearch, SearchStrategy};
use std::sync::mpsc::TryRecvError::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

struct Problem {
    id: usize,
    graph: Arc<RwLock<Graph>>,
    start: (isize, isize),
    end: (isize, isize),
}

pub struct Solution {
    pub id: usize,
    pub path: Option<Vec<(isize, isize)>>,
}

pub struct Solver {
    prob_send: Sender<Option<Problem>>,
    soln_recv: Receiver<Solution>,
    thread_handle: Option<thread::JoinHandle<()>>,
    count: usize,
}

impl Solver {
    pub fn new() -> Solver {
        let (prob_send, prob_recv) = channel::<Option<Problem>>();
        let (soln_send, soln_recv) = channel::<Solution>();

        let h = thread::spawn(move || {
            let prob_recv = prob_recv;
            let soln_send = soln_send;
            let search = AStarSearch::new_diagonal();
            loop {
                match prob_recv.recv() {
                    Ok(maybe_problem) => match maybe_problem {
                        Some(problem) => {
                            let id = problem.id;
                            let path = search.solve(
                                &*problem.graph.read().ok().expect("threading sucks"),
                                problem.start,
                                problem.end,
                            );
                            soln_send
                                .send(Solution { id, path })
                                .expect("Threading still sucks");
                        }
                        None => break,
                    },
                    Err(_) => panic!("ERROR: Solver receiver disconnected"),
                }
            }
        });

        Solver {
            prob_send,
            soln_recv,
            thread_handle: h.into(),
            count: 0,
        }
    }
    pub fn queue_solve(
        &mut self,
        id: usize,
        graph: Arc<RwLock<Graph>>,
        start: (isize, isize),
        end: (isize, isize),
    ) {
        let p = Problem {
            id,
            graph,
            start,
            end,
        };
        self.prob_send
            .send(Some(p))
            .expect("Seriously threading sucks a lot");
        self.count += 1;
    }
    pub fn poll(&mut self) -> Option<Solution> {
        match self.soln_recv.try_recv() {
            Ok(soln) => {
                self.count -= 1;
                Some(soln)
            }
            Err(e) => match e {
                Empty => None,
                Disconnected => panic!("ERROR: Solver task killed prematurely"),
            },
        }
    }
    // pub fn get_problem_count(&self) -> usize {
    //     self.count
    // }
}

impl Drop for Solver {
    fn drop(&mut self) {
        self.prob_send.send(None).expect("ouch threading oof");
        self.thread_handle.take().map(|h| h.join());
    }
}
