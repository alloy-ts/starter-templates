import { expect, test } from "vite-plus/test";
import { establishConnection } from "./main.ts";

test("Diesel ORM Node bindings - posts operations", () => {
  const conn = establishConnection(":memory:");
  conn.setupTables();

  const post1 = conn.createPost("Diesel demo", "Writing a blog demo with Diesel ORM");
  expect(post1.id).toBe(1);
  expect(post1.title).toBe("Diesel demo");
  expect(post1.published).toBe(false);

  // Show posts - unpublished post is not returned when publishedOnly is true
  const publishedBefore = conn.getPosts(true, 5);
  expect(publishedBefore.length).toBe(0);

  // Get post by id
  const fetched = conn.getPost(1);
  expect(fetched).not.toBeNull();
  expect(fetched?.title).toBe("Diesel demo");

  // Publish post
  const publishedPost = conn.publishPost(1);
  expect(publishedPost.published).toBe(true);

  // Now published posts query returns it
  const publishedAfter = conn.getPosts(true, 5);
  expect(publishedAfter.length).toBe(1);
  expect(publishedAfter[0]?.title).toBe("Diesel demo");

  // Delete post
  const deletedCount = conn.deletePost("demo");
  expect(deletedCount).toBe(1);

  const postsRemaining = conn.getPosts();
  expect(postsRemaining.length).toBe(0);
});

test("Diesel ORM Node bindings - users operations", () => {
  const conn = establishConnection(":memory:");
  conn.setupTables();

  const insertedUsers = conn.createUsers([
    { name: "Sean", hairColor: "Black" },
    { name: "Gordon" },
  ]);

  expect(insertedUsers.length).toBe(2);
  expect(insertedUsers[0]?.name).toBe("Sean");
  expect(insertedUsers[0]?.hairColor).toBe("Black");
  expect(insertedUsers[1]?.name).toBe("Gordon");
  expect(insertedUsers[1]?.hairColor).toBeUndefined();

  const allUsers = conn.getUsers();
  expect(allUsers.length).toBe(2);

  // Update batch/filter
  const updatedCount = conn.updateUserBanned("%Gordon%", true);
  expect(updatedCount).toBe(1);

  const updatedUsers = conn.getUsers();
  const gordon = updatedUsers.find((u) => u.name === "Gordon");
  expect(gordon?.banned).toBe(true);
});

test("Diesel ORM Node bindings - downloads and raw SQL", () => {
  const conn = establishConnection(":memory:");
  conn.setupTables();

  const version = conn.createVersion(1, "1.0.0");
  expect(version.id).toBe(1);

  const dl = conn.createDownload(version.id, 100, 100, "2026-01-01");
  expect(dl.downloads).toBe(100);

  const downloads = conn.getDownloads([version.id]);
  expect(downloads.length).toBe(1);
  expect(downloads[0]?.downloads).toBe(100);

  conn.executeRawSql(
    "INSERT INTO posts (title, body, published) VALUES ('Raw SQL Post', 'Body text', 1);",
  );
  const posts = conn.getPosts();
  expect(posts.length).toBe(1);
  expect(posts[0]?.title).toBe("Raw SQL Post");
});
