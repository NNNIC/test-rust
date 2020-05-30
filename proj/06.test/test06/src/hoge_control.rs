#[allow(dead_code)]

pub mod hoge_control {
    #[derive(PartialEq,Clone,Copy)]
    enum State {
        #[allow(non_camel_case_types)]
        None,
        //    [STATEGO OUTPUT START] indent(8) $/^S_/->#enums$
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg
        #[allow(non_camel_case_types)]
        S_0001,
        #[allow(non_camel_case_types)]
        S_0002,
        #[allow(non_camel_case_types)]
        S_END,
        #[allow(non_camel_case_types)]
        S_START,


        //    [STATEGO OUTPUT END]
    }

    //    [STATEGO OUTPUT START] indent(4) $/^E_/$ 
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg


    //    [STATEGO OUTPUT END] 

    pub struct HogeControl {
        m_first: bool,
        m_nowait: bool,
        m_cur:  State,
        m_next: State,

        // ステート関数のmember用 , memberinitもいる！
        //    [STATEGO OUTPUT START] indent(8) $/^S_/->#def_members$
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg
        m_cnt: i32,

        //    [STATEGO OUTPUT END] 
    }

    impl HogeControl {
        pub fn new() -> HogeControl {
            HogeControl {
                m_first: false,
                m_nowait: false,
                m_cur: State::None,
                m_next: State::None,
                //    [STATEGO OUTPUT START] indent(16) $/^S_/->#ini_members$
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg
                m_cnt: 0,

                //    [STATEGO OUTPUT END] 
            }
        }

        pub fn run(&mut self) {
            println!("@run");
            self.start();
            loop {
                self.update();
                if self.is_end() {
                     break;    
                }
            }
            println!("@exit");
        }

        fn start(&mut self) {
            self.m_next = State::S_START;
        }
        fn is_end(&self) -> bool {
            return self.m_cur == State::S_END;
        }
        fn goto(&mut self, s : State) {
            self.m_next = s;
        }
        fn has_next(&self) -> bool {
            self.m_next != State::None
        }
        fn no_wait(&mut self) {
            self.m_nowait = true;
        }

        fn update(&mut self) {
            loop {
                self.m_nowait = false;
                self.m_first = false;
                if self.m_next != State::None {
                    self.m_cur = self.m_next;
                    self.m_next = State::None;
                    self.m_first = true;            
                }
                if self.m_cur != State::None {
                    match self.m_cur {
                        //    [STATEGO OUTPUT START] indent(24) $/^S_/->#matches$
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg
                        State::S_0001 => self.S_0001(),
                        State::S_0002 => self.S_0002(),
                        State::S_END => self.S_END(),
                        State::S_START => self.S_START(),


                        //    [STATEGO OUTPUT END]
                        _ => {},
                    }
                }
                if self.m_nowait == false {
                    break;
                }
            }
        }
        //    [STATEGO OUTPUT START] indent(8) $/^S_/$
//  psggConverterLib.dll converted from hoge_control.xlsx.    psgg-file:hoge_control.psgg
        /*
            S_0001
        */
        #[allow(non_snake_case)]
        fn S_0001(&mut self) {
            if self.m_first {
                println!("StateGo is running");
            }
            if !self.has_next() {
                self.goto(State::S_0002);
            }
        }
        /*
            S_0002
        */
        #[allow(non_snake_case)]
        fn S_0002(&mut self) {
            if self.m_first {
                self.m_cnt = 0;
            }
            println!("cnt={}", self.m_cnt);
            self.m_cnt += 1;
            if self.m_cnt != 10 {
                return;
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_END
        */
        #[allow(non_snake_case)]
        fn S_END(&mut self) {
        }
        /*
            S_START
        */
        #[allow(non_snake_case)]
        fn S_START(&mut self) {
            self.goto(State::S_0001);
            self.no_wait();
        }


        //    [STATEGO OUTPUT END]
    }

}
