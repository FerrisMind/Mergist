export const ACCORDION_KEY = Symbol('accordion');

export type AccordionContext = {
  active: string | null;
  toggle: (v: string) => void;
};
