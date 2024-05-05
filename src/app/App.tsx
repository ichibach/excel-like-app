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

  async function openFileClickHandler(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'ExcelFile',
        extensions: ['xls', 'xlsx']
      }]
    });
    console.log(selected)

    if (selected) {
      await invoke("open_file", {file: selected})
        .then(r => console.log(r))
        .catch(e => console.error(e))
      // user selected multiple files
    } 
  }

  const readFile = async () => {

    const selected = await open({
      multiple: false,
      filters: [{
        name: 'ExcelFile',
        extensions: ['xls', 'xlsx']
      }]
    });
    console.log(selected)


    fs.readBinaryFile(selected as string)
      .then(r => { 


        const workbook = read(r, { cellStyles: true, dense: true });
          // XLSX.read(r)          
          ;

        const first_ws = workbook.Sheets[workbook.SheetNames[1]];

        
        // const st = wb

        setWorkBook(workbook);
        setOpenedFilePath(selected as string);
      })
      .catch(e => console.error(e))

  }


  return (
    <div className="w-screen h-screen overflow-hidden relative">
      {/* <h1>Welcome to Tauri!</h1>

      <button onClick={openFileClickHandler}>Open File</button>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <Button onClick={readFile} >Open File</Button>
      <p>
        {openedFilePath}
      </p>


      <DocumentPage wb={workbook} filePath={openedFilePath}/>
    </div>
  );
}

export default App;
