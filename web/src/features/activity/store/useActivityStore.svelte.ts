import {
  withEntry,
  type ActivityDirectionType,
  type ActivityEntryType,
} from '../logic/activity';

const LOG_LIMIT = 200;

class ActivityStore {
  entries = $state<ActivityEntryType[]>([]);
  #nextId = 0;

  record = (direction: ActivityDirectionType, text: string) => {
    const time = new Date().toLocaleTimeString([], { hour12: false });
    const entry = { id: this.#nextId++, time, direction, text };
    this.entries = withEntry(this.entries, entry, LOG_LIMIT);
  };
}

const store = new ActivityStore();

export function useActivityStore() {
  return store;
}
