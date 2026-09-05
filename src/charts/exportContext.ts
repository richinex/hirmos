import { createContext, useContext } from 'react'

/**
 * What a chart export is about beyond the chart itself: the project it came from. The workbench
 * provides it once; every floating window folds it into the file name, so a folder of downloads
 * still says which analysis each drawing belongs to.
 */
export interface ChartExportContext {
  readonly project: string | null
}

export const ChartExportProvider = createContext<ChartExportContext>({ project: null })

export const useChartExportContext = (): ChartExportContext => useContext(ChartExportProvider)
