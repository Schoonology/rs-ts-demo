import { atom } from "recoil";

export type Post = {
  message: string;
  timestamp: number;
};

export const aPosts = atom<Array<Post>>({
  key: "posts",
  effects: [
    ({ setSelf }) => {
      fetch("/posts")
        .then((response) => response.json())
        .then((json) => {
          setSelf(json);
        });
    },
    ({ setSelf }) => {
      const stream = new EventSource("/updates");

      stream.onmessage = ({ data, type }) => {
        switch (type) {
          case "message":
            setSelf((existing) => {
              if (Array.isArray(existing)) {
                return [...existing, JSON.parse(data)];
              } else {
                return existing;
              }
            });
            break;
        }
      };
    },
  ],
});
