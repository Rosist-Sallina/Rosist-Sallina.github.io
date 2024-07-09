use yew::prelude::*;
use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::builtin_words::select;
use crate::judge::crate_judge;
use crate::resouces::_dmode_vavid_check;
use std::collections::HashMap;

pub struct Model {
    show_menu: bool,
    is_difficult:bool,
    seed:String,
    day:String,
    inputs: Vec<Vec<String>>,
    current_row: usize,
    all_submitted : bool,
    colors: Vec<Vec<String>>,
    answer: String,
    result: String,
    input : String,
    all_completed : bool,
    alphabet_color: String,
    char_color : HashMap<char , char>,
    total_round:i32,
    total_success:i32,
    rounds:i32,
    average_round:f32,
    words:HashMap<String , i32>,
    total_success_rounds:i32,
}

pub enum Msg {
    ToggleMenu,
    SelectValue(String),
    ClickOutside,
    _Clickwithnothing,
    DifficultCheck,
    UpdateSeed(String),
    UpdateDay(String),
    UpdateInput(usize,usize,String),
    EnableNextRow,
    FocusNext((usize, usize)),
    ClearRow(usize),
    FocusPrevious((usize, usize)),
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let inputs = vec![vec![String::new(); 5]; 6];
        let colors = vec![vec!["".to_string(); 5]; 6];
        let alphabet_color = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();

        let onclick = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
            link.send_message(Msg::ClickOutside);
        }) as Box<dyn FnMut(_)>);

        body.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref()).unwrap();
        onclick.forget();

        Self {
            show_menu: false,
            is_difficult:false,
            seed:"114514".to_string(),
            day:"810".to_string(),
            inputs,
            current_row: 0,
            all_submitted: false,
            colors,
            answer:String::from("TITAN"),
            result: String::from(""),
            input: String::from(""),
            all_completed : false,
            alphabet_color,
            char_color : HashMap::new(),
            total_round:0,
            total_success:0,
            total_success_rounds:0,
            rounds:0,
            words:HashMap::new(),
            average_round:0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.show_menu = !self.show_menu;
            }
            Msg::_Clickwithnothing=>{       
            }
            Msg::SelectValue(value) => {
                self.show_menu = false;
                info!("Selected value: {}", value);
            }
            Msg::ClickOutside => {
                if self.show_menu {
                    self.show_menu = false;
                    return true;
                }
            }
            Msg::DifficultCheck =>{
                self.is_difficult = !self.is_difficult;       
            }
            Msg::UpdateSeed(value) =>{
                self.seed = value.parse().unwrap();
                let mut seed = 114514;
                match self.seed.parse::<u64>() {
                    Ok(value) => {
                        seed = value;
                    },
                    Err(_e) => {
                        seed = 114514;
                    }
                }
                let mut day = 810;
                match self.day.parse::<usize>(){
                    Ok(value) =>{
                        day = value;
                    },
                    Err(_e) =>{
                        day = 810;
                    }
                }
                self.answer = crate::resouces::get_useable_word_default(day, seed);
            }
            Msg::UpdateDay(value) =>{
                self.day = value.parse().unwrap();
                let mut seed = 114514;
                match self.seed.parse::<u64>() {
                    Ok(value) => {
                        seed = value;
                    },
                    Err(_e) => {
                        seed = 114514;
                    }
                }
                let mut day = 810;
                match self.day.parse::<usize>(){
                    Ok(value) =>{
                        day = value;
                    },
                    Err(_e) =>{
                        day = 810;
                    }
                }
                self.answer = crate::resouces::get_useable_word_default(day, seed);
            }
            Msg::UpdateInput(row, col, value) => {
                let value = value.to_uppercase();  // 转换为大写
                if row == self.current_row && !self.all_submitted {
                    if let Some(input_row) = self.inputs.get_mut(row) {
                        if let Some(input) = input_row.get_mut(col) {
                            let is_empty = value.is_empty();
                            *input = value.clone();
                            if is_empty && col > 0 {
                                ctx.link().send_message(Msg::FocusPrevious((row, col - 1)));
                            } else if !is_empty && value.len() == 1 && col < 4 {
                                ctx.link().send_message(Msg::FocusNext((row, col + 1)));
                            }
                        }
                    }
                }
            }
            Msg::EnableNextRow => {
                // 进行合法性检查
                if !self.validate_row(self.current_row) {
                    return false;
                }
                let (result, input , alphabet_result , char_color) = crate_judge::judge(self.inputs[self.current_row].clone() , self.answer.clone().as_str() , self.char_color.clone());
                if self.is_difficult{
                    if !_dmode_vavid_check(self.input.as_str(), &input, &self.result){
                        return false;
                    }
                }
                let value = self.words.entry(input.clone()).or_insert(0);
                    *value += 1;
                self.result = result.clone();
                self.input = input.clone();
                self.char_color = char_color.clone();
                let mut all_green = true;
                for (col, color) in result.chars().enumerate() {
                    self.colors[self.current_row][col] = match color {
                        'G' => "green".to_string(),
                        'Y' => "#CCCC00".to_string(),
                        'R' => "red".to_string(),
                        _ => "white".to_string(),
                    };
                    if color != 'G' {
                        all_green = false;
                    }
                }
                self.rounds += 1;
                if all_green {
                    self.total_success += 1;
                    self.total_round += 1;
                    self.total_success_rounds += self.rounds;
                    self.words = crate::resouces::hash_map_sort(self.words.clone());
                    self.average_round = self.total_success_rounds as f32 / self.total_success as f32;
                    self.all_completed = true;
                    self.rounds = 0;
                    self.day = (self.day.parse::<usize>().unwrap() + 1).to_string();
                    self.answer = crate::resouces::get_useable_word_default(self.day.parse::<usize>().unwrap(), self.seed.parse::<u64>().unwrap());
                    self.result = String::from("");
                    self.input = String::from("");
                    self.alphabet_color= "XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
                    let link = ctx.link().clone();
                    let handle = move || {
                        link.send_message(Msg::Reset);
                        web_sys::window().unwrap().alert_with_message("Congratulations!The game will reset.").unwrap();
                    };
                    self.inputs = vec![vec![String::new(); 5]; 6];
                    self.colors = vec![vec!["".to_string(); 5]; 6];
                    self.current_row = 0;
                    self.all_completed = false;
                    let _ = wasm_bindgen_futures::spawn_local(async move {
                        gloo::timers::future::TimeoutFuture::new(100).await; // 500ms 延迟
                        handle();
                    });
                    self.char_color = HashMap::new();
                }else if !all_green && self.current_row == 5{
                    self.total_round += 1;
                    self.rounds = 0;
                    self.day = (self.day.parse::<usize>().unwrap() + 1).to_string();
                    self.answer = crate::resouces::get_useable_word_default(self.day.parse::<usize>().unwrap(), self.seed.parse::<u64>().unwrap());
                    self.result = String::from("");
                    self.input = String::from("");
                    self.alphabet_color= "XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
                    let link = ctx.link().clone();
                    let handle = move || {
                        link.send_message(Msg::Reset);
                        web_sys::window().unwrap().alert_with_message("ざこお兄ちゃ～～ The game will reset.").unwrap();
                    };
                    self.inputs = vec![vec![String::new(); 5]; 6]; 
                    self.colors = vec![vec!["".to_string(); 5]; 6];
                    self.current_row = 0;
                    self.all_completed = false;
                    let _ = wasm_bindgen_futures::spawn_local(async move {
                        gloo::timers::future::TimeoutFuture::new(100).await; // 500ms 延迟
                        handle();
                    });
                    self.char_color = HashMap::new();
                }
                self.alphabet_color = alphabet_result.clone();
                if !self.all_submitted && self.current_row < self.inputs.len() && self.inputs[self.current_row].iter().all(|s| !s.is_empty()) {
                    self.current_row += 1;
                    if self.current_row == self.inputs.len() {
                        self.all_submitted= true;
                    }
                }
            }
            Msg::FocusNext((row, col)) => {
                let input_id = format!("input-{}-{}", row, col);
                if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                    if let Some(element) = document.get_element_by_id(&input_id) {
                        if let Some(input) = element.dyn_into::<HtmlInputElement>().ok() {
                            input.focus().unwrap();
                        }
                    }
                }
            }
            Msg::FocusPrevious((row, col)) => {
                let input_id = format!("input-{}-{}", row, col);
                if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                    if let Some(element) = document.get_element_by_id(&input_id) {
                        if let Some(input) = element.dyn_into::<HtmlInputElement>().ok() {
                            input.focus().unwrap();
                        }
                    }
                }
            }
            Msg::ClearRow(row)=>{
                if let Some(input_row) = self.inputs.get_mut(row) {
                    for input in input_row.iter_mut() {
                        *input = String::new();
                    }
                }
            }
            Msg::Reset => {
                self.alphabet_color = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
            }
        }
        true
    }

    fn changed(&mut self, _: &Context<Self>, _: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_style = if self.show_menu {
            "opacity: 1; visibility: visible; transition: opacity 0.5s ease-in-out; \
             width: 200px; border-radius: 5px; background: #f9f9f9; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);"
        } else {
            "opacity: 0; visibility: hidden; transition: opacity 0.5s ease-in-out; \
             width: 200px; border-radius: 5px; background: #f9f9f9; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);"
        };
    
        html! {
            <div>
                //顶部中心图片
                <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 2000;">
                    <a href="/">
                        <img src="https://vip.helloimg.com/i/2024/07/09/668c11135062e.png" alt="Top Center Image" style="width: 200px; height: auto;" />
                    </a>
                </div>
                <hr style="position: fixed; top: 80px; left: 0; border: 0; border-top: 2px solid #ccc; width: 100%; z-index: 999;" />
                //菜单按钮和config
                <div style="position: fixed; top: 30px; left: 20px; z-index: 2000;">
                    <button style="background: none; border: none; cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                        e.stop_propagation();
                        Msg::ToggleMenu
                    })}>
                        <img src="https://vip.helloimg.com/i/2024/07/08/668be3bc35970.png" alt="Settings" style="width: 45px; height: 24px;" />
                    </button>
                    <ul style={menu_style}>
                        <li style="padding: 8px; margin-bottom: 3px;cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::SelectValue("Difficult: ".into())
                        })}>{ "Difficult " }
                        <button onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::DifficultCheck
                        })} style="margin-left: 5px; background: none; border: none; cursor: pointer; font-size: 19px; margin-top: -2px">
                            { if self.is_difficult { "☑" } else { "☐" } }
                        </button>
                        </li>
                        <li style="padding: 8px; cursor: pointer;">
                            { "Seed: " }
                            <input
                                type="text"
                                value={self.seed.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateSeed(input.value())
                                })}
                                onclick={ctx.link().callback(|e: MouseEvent| {
                                    e.stop_propagation();
                                    Msg::_Clickwithnothing
                                })}
                                style="margin-left: 10px; padding: 4px; font-size: 16px; width: 100px;"
                            />
                        </li>
                        <li style="padding: 8px; cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::SelectValue("Day: ".into())
                        })}>{ "Day: " }<input
                            type="text"
                            value={self.day.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateDay(input.value())
                            })}
                            onclick={ctx.link().callback(|e: MouseEvent| {
                                e.stop_propagation();
                                Msg::_Clickwithnothing
                            })}
                            style="margin-left: 10px; padding: 4px; font-size: 16px; width: 100px;"
                        />
                    </li>
                    </ul>
                </div>
                //主要输入框
                <div style="position: fixed; top: 150px; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 10px; z-index: 1500;">
                    { (0..6).map(|row| self.view_row(ctx, row)).collect::<Html>() }
                </div>
                <div style="position: fixed; bottom: 50px; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 10px; z-index: 1500;">
                    { self.view_keyboard() }
                    <div>
                        <button style="position: relative; bottom: 133px; left: 409px; width: 136px; font-size: 28px;height: 68px; background-color: gray; color: balck; border: none; border-radius: 5px;" onclick={ctx.link().callback(|_| Msg::EnableNextRow)} disabled={self.all_submitted || self.inputs[self.current_row].iter().any(|s| s.is_empty())}>
                                { "ENTER" }
                        </button>
                    </div>
                </div>
                <div style="
                    position: fixed;
                    top: 200px;
                    left: 30px;
                    background: rgba(242, 237, 237, 0.8);  // 降低透明度
                    border: 1px solid #ccc;
                    padding: 10px;
                    border-radius: 10px;  // 圆角效果
                    box-shadow: 0 4px 8px rgba(5, 3, 3, 0.3);  // 添加阴影效果
                    z-index: 2000;
                    backdrop-filter: blur(5px);  // 添加模糊效果
                    -webkit-backdrop-filter: blur(5px);  // 添加模糊效果 (兼容WebKit)">
                    <p>{ format!("Total Rounds: {}", self.total_round) }</p>
                    <p>{ format!("Total Success: {}", self.total_success) }</p>
                    <p>{ format!("Average Round: {:.2}", self.average_round) }</p>
                    <ul>
                        { for self.words.iter().take(5).map(|(word, &count)| html! {
                            <li>{ format!("{}: {}", word, count) }</li>
                        })}
                    </ul>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_row(&self, ctx: &Context<Self>, row: usize) -> Html {
        let is_disabled = row > self.current_row || self.all_submitted;
        let is_clear_disabled = row != self.current_row || self.all_submitted;
        html! {
            <div style="display: flex; gap: 5px; justify-content: center;">
                { (0..5).map(|col| self.view_input(ctx, row, col, is_disabled)).collect::<Html>() }
                <button style="width: 68px; height: 68px; padding: 3px; font-size: 20px; text-align: center; border: 1px solid #ccc; border-radius: 3px;"
                        onclick={ctx.link().callback(move |_| Msg::ClearRow(row))} disabled={is_clear_disabled}>
                    { "Clear" }
                </button>
            </div>
        }
    }

    fn view_input(&self, ctx: &Context<Self>, row: usize, col: usize, is_disabled: bool) -> Html {
        let input_id = format!("input-{}-{}", row, col);
        let color = self.colors[row][col].clone();
        let text_color = if color == "green" || color == "#CCCC00" || color == "red" {
            "white"
        } else {
            "black"
        };
        html! {
            <input
                id={input_id}
                type="text"
                value={self.inputs[row][col].clone()}
                oninput={ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value().to_uppercase();  // 转换为大写
                    Msg::UpdateInput(row, col, value)
                })}
                disabled={is_disabled}
                style={format!("width: 60px; height: 60px; padding: 3px; font-size: 28px; text-align: center; border: 1px solid #ccc; border-radius: 3px; background-color: {}; color: {};", color, text_color)}
                maxlength="1"
            />
        }
    }
    fn view_keyboard(&self) -> Html {
        let keys_rows = vec![
            "QWERTYUIOP",
            "ASDFGHJKL",
            "ZXCVBNM",
        ];
        let key_colors = self.alphabet_color.chars().collect::<Vec<_>>();
        
        html! {
        <div style=" display: flex; position:fixed; bottom: 50px; flex-direction: column; align-items: center; gap: 5px; width: 100%; padding: 10px;">
            { for keys_rows.iter().enumerate().map(|(_row_index, row)| {
                html! {
                    <div style="display: flex; justify-content: center; gap: 5px;">
                        { for row.chars().enumerate().map(|(_col_index, key)| {
                            let color = match key_colors[key as usize - 'A' as usize] {
                                'G' => "green",
                                'Y' => "#CCCC00",
                                'R' => "red",
                                _ => "gray",
                            };
                            let text_color = if color == "green" || color == "#CCCC00" || color == "red" {
                                "white"
                            } else {
                                "black"
                            };
                            html! {
                                <div style={format!("width: 68px; height: 68px; font-size: 28px; display: flex; align-items: center; justify-content: center; background-color: {}; color: {}; border-radius: 5px;", color, text_color)}>
                                    { key }
                                </div>
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
    }


    fn validate_row(&self, row: usize) -> bool {
        let temp_str = self.inputs[row].iter().fold(String::new(), |acc, s| acc + s);
        let temp_str = temp_str.to_lowercase();
        if !select::ACCEPTABLE.contains(&temp_str.as_str()) || !self.inputs[row].iter().all(|s| s.chars().all(|c| c.is_ascii_alphabetic())) {
            return false;
        }
        else{
            true
        }
    }
}