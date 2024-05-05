
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/shared/components/ui/tabs";
import { SpreadSheetWidget } from "@/widgets/SpreadSheetWidget";
import { GridCell, GridCellKind, Item } from "@glideapps/glide-data-grid";
import "@glideapps/glide-data-grid/dist/index.css";
import { useCallback, useEffect, useState } from "react";
// import Spreadsheet from "react-spreadsheet";
import { WorkBook } from "xlsx";


export interface DocumentPageProps {
  wb: WorkBook | null;  
  filePath: string | null;  
}


export const DocumentPage = (props: DocumentPageProps) => {
  const { wb, filePath } = props;

  useEffect(() => {
    // setDefaultTabValue(filePath || undefined);
  },[filePath])

  if(!wb) return null;

  return (
    <Tabs 
      defaultValue={wb?.SheetNames[0]}
      tabIndex={0}
      className="w-screen"  
    >
      <div className="w-full h-[92vh] overflow-scroll">
        { wb.SheetNames.map((name) => (
            <TabsContent value={name}>
              <SpreadSheetWidget wb={wb} spreadSheetName={name}/>
            </TabsContent>
          ))
        }
      </div>
      <TabsList className="fixed left-0 bottom-0">
        { wb.SheetNames.map((name) => (
            <TabsTrigger value={name}>{name}</TabsTrigger>
          ))
        }
      </TabsList>
    </Tabs>
  )
}