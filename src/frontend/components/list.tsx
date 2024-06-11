import { useRecoilValue } from "recoil";
import { type Post, aPosts } from "../state";
import React from "react";

function Message(post: Post) {
  return (
    <li className="border-l-4 border-cyan-400 my-3 pl-3 py-1">
      <p className="font-bold text-cyan-950 text-lg">{post.message}</p>
      <p className="opacity-65 text-sm">
        Posted at {new Date(post.timestamp).toISOString()}
      </p>
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
