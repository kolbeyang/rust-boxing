import { IconArrowLeft } from "@tabler/icons-react";
import { useNavigate } from "react-router";

import { cn } from "../../utils/classNameMerge";

interface Props {
  goBack?: () => void;
}

const TopBar = ({ goBack }: Props) => {
  const navigate = useNavigate();
  return (
    <div className="flex w-full gap-[8px]">
      {goBack && (
        <button
          className={cn(
            "rounded-[4px] bg-gray-200 p-2 tracking-widest",
            "hover:bg-gray-300 active:bg-gray-400",
          )}
          onClick={goBack}
        >
          <IconArrowLeft stroke={1.5} />
        </button>
      )}
      <button
        className={cn(
          "rounded-[4px] bg-gray-200 px-3 font-family-shoulders font-stretch-extra-condensed font-semibold min-h-[40px] text-[20px] justify-center items-center",
          "hover:bg-gray-300 active:bg-gray-400 ",
        )}
        onClick={() => navigate("/")}
      >
        RUST BOXING
      </button>
    </div>
  );
};

export default TopBar;
