import { useRecoilValue } from "recoil";
import { type Post, aPosts } from "../state";
import React from "react";

function Message(post: Post) {
  return (
    <li>
      <p>{post.message}</p>
      <p>{post.timestamp}</p>
    </li>
  );
}

export function List() {
  const posts = useRecoilValue(aPosts);

  return (
    <ul className="m-3">
      {posts.map((post) => (
        <Message key={post.timestamp} {...post}></Message>
      ))}
    </ul>
  );
}
