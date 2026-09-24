/**
 * ECharts for tests that read or drive a mounted chart.
 *
 * Vite resolves this import, so no test has to name the optimised-dependency path. That path carries
 * a hash Vite mints each time it rebuilds dependencies, and it refuses the path without the current
 * hash, so a test that hard-codes one fails as soon as dependencies are rebuilt.
 */
export { getInstanceByDom } from 'echarts/core'
