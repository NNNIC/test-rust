#[allow(dead_code)]

mod hoge_control {
    #[derive(PartialEq,Clone,Copy)]
    enum State {
        None,
        Sstart,
        Shoge,
        Send,
    }
    pub struct HogeControl {
        m_first: bool,
        m_nowait: bool,
        m_cur: State,
        m_next: State,

        // ステート関数のmember用 , memberinitもいる！
        m_cnt: i32,
    }

    impl HogeControl {
        pub fn new() -> HogeControl {
            HogeControl {
                m_first: false,
                m_nowait: false,
                m_cur: State::None,
                m_next: State::None,
                m_cnt: 0,
            }
        }

        pub fn run(&mut self) {
            self.start();
            loop {
                self.update();
                if self.is_end() {
                     break;    
                }
            }
        }

        fn start(&mut self) {
            self.m_next = State::Sstart;
        }
        fn is_end(&self) -> bool {
            return self.m_cur == State::Send;
        }
        fn goto(&mut self, s : State) {
            self.m_next = s;
        }
        fn has_next(&self) -> bool {
            self.m_next != State::None
        }

        fn update(&mut self) {
            self.m_nowait = false;
            loop {
                self.m_first = false;
                if self.m_next != State::None {
                    self.m_cur = self.m_next;
                    self.m_next = State::None;
                    self.m_first = true;            
                }
                if self.m_cur != State::None {
                    match self.m_cur {
                        State::Sstart => self.sstart(),
                        State::Send   => self.send(),
                        State::Shoge  => self.shoge(),
                        _ => {},
                    }
                }
                if self.m_nowait == false {
                    break;
                }
            }
        }

        fn sstart(&mut self) {
            if self.m_first {
                println!("START!");
                self.goto(State::Shoge);
            }
        }

        fn shoge(&mut self) {
            if self.m_first {
                self.m_cnt  = 0;
                return;
            }
            println!("counter = {}", self.m_cnt);
            self.m_cnt+=1;
            if self.m_cnt == 10 {
                self.goto(State::Send);        
            }
        }

        fn send(&mut self) {
            println!("END!");
        }
    }
}

fn main() {
    let mut hc = hoge_control::HogeControl::new();
    hc.run();
}
