import {
  DbConnection as NativeDbConnection,
  type JsDownload,
  type JsNewUser,
  type JsPost,
  type JsUser,
  type JsVersion,
} from "../index.js";

export type {
  JsDownload as Download,
  JsNewUser as NewUser,
  JsPost as Post,
  JsUser as User,
  JsVersion as Version,
};

export class DbConnection {
  private native: NativeDbConnection;

  constructor(databaseUrl: string) {
    this.native = new NativeDbConnection(databaseUrl);
  }

  setupTables(): void {
    this.native.setupTables();
  }

  createPost(title: string, body: string): JsPost {
    return this.native.createPost(title, body);
  }

  getPosts(publishedOnly?: boolean, limit?: number): JsPost[] {
    return this.native.getPosts(publishedOnly, limit);
  }

  getPost(postId: number): JsPost | null {
    return this.native.getPost(postId);
  }

  publishPost(postId: number): JsPost {
    return this.native.publishPost(postId);
  }

  deletePost(targetTitleSubstring: string): number {
    return this.native.deletePost(targetTitleSubstring);
  }

  createUsers(newUsers: JsNewUser[]): JsUser[] {
    return this.native.createUsers(newUsers);
  }

  getUsers(): JsUser[] {
    return this.native.getUsers();
  }

  updateUserBanned(pattern: string, isBanned: boolean): number {
    return this.native.updateUserBanned(pattern, isBanned);
  }

  executeRawSql(sql: string): number {
    return this.native.executeRawSql(sql);
  }

  createVersion(crateId: number, num: string): JsVersion {
    return this.native.createVersion(crateId, num);
  }

  createDownload(versionId: number, downloads: number, counted: number, date: string): JsDownload {
    return this.native.createDownload(versionId, downloads, counted, date);
  }

  getDownloads(versionIds: number[]): JsDownload[] {
    return this.native.getDownloads(versionIds);
  }
}

export function establishConnection(databaseUrl: string): DbConnection {
  return new DbConnection(databaseUrl);
}

export const establish_connection = establishConnection;
