import { load } from '@tauri-apps/plugin-store';

export class HistoryManager {
    private storePromise;

    constructor() {
        this.storePromise = load('history.json', { autoSave: true });
    }

    async createEntry(url: string): Promise<void> {
        const store = await this.storePromise;
        const currentHistory = await store.get<string[]>('histStorage') ?? [];
        currentHistory.push(url);
        await store.set('histStorage', currentHistory);
    }

    async getEntries(): Promise<string[]> {
        const store = await this.storePromise;
        return await store.get<string[]>('histStorage') ?? [];
    }
}
