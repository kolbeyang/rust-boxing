import type { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

const Layout = ({ children }: Props) => {
  return (
    <div className="h-screen w-screen text-sm md:text-base">{children}</div>
  );
};

export default Layout;
