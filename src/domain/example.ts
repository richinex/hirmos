import type { SavedProjectHeader } from '@/domain/persistence'

/** The shipped Seatbelts walkthrough, a project bundle with its source file inside. */
export const EXAMPLE_BUNDLE_URL = '/examples/seatbelts.hirmos.json'

/** The project id written into the shipped bundle, so the saved copy is recognised whatever build produced it. */
export const EXAMPLE_PROJECT_ID = 'd95e0c7b-44ec-4dee-a929-784cfd923eeb' as SavedProjectHeader['id']
export const EXAMPLE_PROJECT_NAME = 'Seat-belt law and road deaths'
export const EXAMPLE_SOURCE_NAME = 'Seatbelts.csv'
