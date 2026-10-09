/**
 * React Flow listens for Space on the whole document to pan and cancels the key press, which stops
 * Space toggling checkboxes, radios and disclosures elsewhere on the page. Dragging still pans.
 */
export const FLOW_DEFAULTS = { panActivationKeyCode: null } as const
