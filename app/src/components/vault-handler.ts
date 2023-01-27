import * as uuid from "uuid";
import * as tauri from "@tauri-apps/api";
import path from "path";

interface Vault
{
    vaultId: string;
    userId: string;
    path: string;
    name: string;
    data: Buffer;
};

class VaultHandler
{
    /*public async createVaultPath(userId: string): Promise<Vault>
    {
        let dataDirectory: string = await tauri.path.appDataDir();
        let vaultId: string = uuid.v4();

        let vault: Vault = {
            vaultId: vaultId,
            userId: userId,
            path: path.join(dataDirectory, userId, vaultId)
        };

        return vault;
    };*/

    public async generateVault(userId: string, vaultName: string): Promise<Vault>
    {
        let dataDirectory: string = await tauri.path.appDataDir();
        let vaultId: string = uuid.v4();

        let vault: Vault = {
            vaultId: vaultId,
            userId: userId,
            path: path.join(dataDirectory, userId, vaultId),
            name: vaultName,
            data: Buffer.alloc(0)
        };

        return vault;
    };

    public async getVaultPath(userId: string, vaultId: string): Promise<string>
    {
        let dataDirectory: string = await tauri.path.appDataDir();

        return path.join(dataDirectory, userId, vaultId);
    };
}