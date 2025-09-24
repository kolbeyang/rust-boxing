
import type { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

const Layout = ({ children }: Props) => {
  return (
    <div className="h-screen w-screen">{children}</div>
  );
};

export default Layout;
