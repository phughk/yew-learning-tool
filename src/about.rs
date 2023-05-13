use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="about">
            <p>{"This tool is used to learn by quizzing"}</p>
            <p>
                {"The frontend is written in Rust WebAssembly using"}
                <a href={"https://yew.rs/"}>{"yew"}</a>{"."}
            </p>
            <p>{"The purpose of the project is to learn things quickly that are otherwise tedious to learn."}</p>
            <p>{"For example"}</p>
            <ul>
                <li>{"languages"}</li>
                <li>{"concepts (from programming or other)"}</li>
                <li>{"statistics and other constants"}</li>
                <li>{"properties (such as Keplar's laws)"}</li>
                <li>{"patterns - design patterns or other (ways of writing code, ways of structuring data, ways of writing a document)"}</li>
                <li>{"geography"}</li>
                <li>{"... literally anything that is a boring dataset that is nice to memorise"}</li>
            </ul>
        </div>
    }
}
