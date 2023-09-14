import React, { JSX } from "react";

function Stack(props: { children: JSX.Element[] }) {
  return <div className="stack">{props.children}</div>;
}

function UserCard(props: { name: string; bio: string }) {
  return (
    <div className="user-card">
      <h2>{props.name}</h2>
      <p>{props.bio}</p>
    </div>
  );
}

export function App() {
  return (
    <Stack>
      <UserCard name="Dom" bio="Street racer and Corona lover" />
      <UserCard name="Jakob" bio="Super spy and Dom's secret brother" />
    </Stack>
  );
}

console.log(
  <Stack>
    <UserCard name="Dom" bio="Street racer and Corona lover" />
    <UserCard name="Jakob" bio="Super spy and Dom's secret brother" />
  </Stack>
);
