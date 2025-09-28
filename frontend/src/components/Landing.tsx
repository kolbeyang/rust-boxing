import { useNavigate } from "react-router";

const Landing = () => {
  const navigate = useNavigate();

  return (
    <div className="flex size-full flex-col items-center justify-center gap-8">
      <div className="rounded-[4px] bg-gray-200 px-4 py-3 leading-[80px] font-family-shoulders font-stretch-extra-condensed font-semibold text-[80px]">
        RUST BOXING
      </div>
      <span className="max-w-[440px] text-center">
        Little AI boxers trained through reinforcement learning in{" "}
        <a
          href="https://burn.dev/"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:text-red-500 underline"
        >
          Burn
        </a>
        , competing live in your browser with{" "}
        <a
          href="https://www.rust-lang.org/"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:text-red-500 underline"
        >
          Rust
        </a>{" "}
        and{" "}
        <a
          href="https://webassembly.org/"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:text-red-500 underline"
        >
          WebAssembly
        </a>
        .
      </span>

      <button
        className="bg-zinc-700 px-8 py-4 text-white rounded-[4px] relative overflow-hidden"
        onClick={() => navigate("/home")}
      >
        <div className="absolute top-0 left-0 size-full animate-blink bg-[#FF3131] z-0" />
        <span className="z-20 relative">FIGHT</span>
      </button>

      <span className="max-w-[400px] text-center fixed bottom-10">
        by{` `}
        <a
          href="https://kolbeyang.com"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:text-red-500 underline"
        >
          Kolbe Yang
        </a>
      </span>
    </div>
  );
};

export default Landing;
