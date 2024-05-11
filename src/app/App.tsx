//@ts-nocheck
import { useState } from "react";
import reactLogo from "@/shared/assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import "./globals.css"
import { Button } from "@/shared/components/ui/button";
import { fs } from "@tauri-apps/api";
import { WorkBook, read } from "xlsx";

import * as XLSX from "xlsx-js-style";
import { DocumentPage } from "@/pages/document-page/DocumentPage";


function App() {
  const [workbook, setWorkBook] = useState<WorkBook| null>(null);
  const [openedFilePath, setOpenedFilePath] = useState<string|null>(null);

  
  const readFile = async() => {
    console.log(123);
    await invoke('set_cell_value', {key: "A1", value: 42})
    await invoke('set_cell_value', {key: "B1", value: 42.3})
    await invoke('set_cell_value', {key: "B2", value: -42})
  

    await invoke('get_range', {range: "A1:B2"})
      .then((d) => console.log (d))
      .catch((e) => console.error(e))
  }

  return (
    <div className="w-screen h-screen overflow-hidden relative">
      {/* <h1>Welcome to Tauri!</h1>

      <button onClick={openFileClickHandler}>Open File</button>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <Button onClick={readFile} >Open File</Button>
      {/* <Button onClick={openFileClickHandler} >Open File Rust</Button> */}
      <p>
        {openedFilePath}
      </p>


      <DocumentPage wb={workbook} filePath={openedFilePath}/>
    </div>
  );
}

export default App;
