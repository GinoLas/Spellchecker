import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {

  const [input, setInput] = useState("");
  const [results,setResults] = useState([])

  useEffect(()=>{
    console.log(input)
    console.log(results)
  },[input,results])

  async function process(currentValue) {
    setResults(await invoke("process", { input : currentValue}));
  }

  function handleChange(e){
    setInput(e)
    process(e)
  }

  return (
    <main className="container">
      <h1>Spellchecker</h1>

      

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="input"
          onChange={(e) => handleChange(e.currentTarget.value)}
          placeholder="Start typing..."
        />
      </form>
      
    </main>
    
  );
}

export default App;
