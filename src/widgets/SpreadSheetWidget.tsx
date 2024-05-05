import { useEffect, useState } from "react";
import { WorkBook } from "xlsx"
import Spreadsheet, { CellBase, Matrix } from "react-spreadsheet";

export interface SpreadSheetWidgetProps {
  wb: WorkBook;
  spreadSheetName: string;
}


export const SpreadSheetWidget = (props: SpreadSheetWidgetProps) => {
  const {wb, spreadSheetName} = props;
  const [data, setData] = useState<Matrix<CellBase>>([]);

  useEffect(() => {
    const sheet = wb.Sheets[spreadSheetName];

    const new_data = sheet["!data"]?.map(row => row.map(cell => ({value: cell.v})));

    setData(new_data || []);
  },[wb, spreadSheetName])

  return (
    <Spreadsheet
      data={data}
    />
  )


}