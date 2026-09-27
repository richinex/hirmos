import { expect, test } from '@playwright/test'
import { NAMED_IN_FULL, namesFigure, namesInProse } from '../src/lib/format/names'

/** Long variable lists are counted so a figure or a sentence stays readable; short ones are named. */

const names = (count: number) => Array.from({ length: count }, (_, index) => `v${index + 1}`)

test('a short list is named in full, as a figure and in a sentence', () => {
  expect(namesFigure(names(NAMED_IN_FULL), 'variables')).toEqual({ value: 'v1, v2, v3, v4, v5', preview: null })
  expect(namesInProse(names(2), (count) => `${count} variables`)).toBe('v1, v2')
})

test('a long list is counted, with its first names as a preview', () => {
  expect(namesFigure(names(82), 'variables')).toEqual({ value: '82 variables', preview: 'v1, v2, v3 and 79 more' })
  expect(namesInProse(names(NAMED_IN_FULL + 1), (count) => `the ${count} variables`)).toBe('the 6 variables')
})
