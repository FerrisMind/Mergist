export const TABS_KEY = Symbol('tabs');

export type TabsContext = {
  value: string;
  setValue: (v: string) => void;
};
