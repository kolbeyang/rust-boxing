import { useNavigate } from "react-router";
import { motion } from "framer-motion";

import { cn } from "../utils/classNameMerge";

const Landing = () => {
  const navigate = useNavigate();

  const words = [
    { text: "Little", delay: 0 },
    { text: "AI", delay: 0.3 },
    { text: "boxers", delay: 0.6 },
  ];

  return (
    <div className="flex size-full flex-col items-center justify-center gap-8 ">
      <div
        className={cn(
          "rounded-[4px] bg-zinc-200 px-4 py-3 text-[60px] leading-[60px] font-family-shoulders font-stretch-extra-condensed font-semibold ",
          "md:px-4 md:py-3 md:text-[80px] md:leading-[80px] font-family-shoulders font-stretch-extra-condensed font-semibold ",
        )}
      >
        RUST BOXING
      </div>
      <span className="w-[360px] max-w-full md:w-[460px] text-center">
        {words.map((word, index) => (
          <>
            <motion.span
              key={word.text}
              animate={{
                fontWeight: [400, 700, 700, 400, 400],
                color: ["#d4d4d8", "#ef4444", "#ef4444", "#dc2626", "#3f3f46"],
              }}
              transition={{
                duration: 0.1,
                delay: word.delay,
                times: [0, 0.01, 0.5, 0.51, 1],
                ease: "linear",
              }}
            >
              {word.text}
              {index < words.length - 1 && " "}
            </motion.span>
            {` `}
          </>
        ))}
        <motion.span
          style={{ display: "inline-block" }}
          initial={{
            opacity: 0,
            translateY: "100px",
          }}
          animate={{
            opacity: 1,
            translateY: "0",
          }}
          transition={{
            duration: 0.05,
            delay: 1.0,
            ease: "linear",
          }}
        >
          trained through reinforcement learning in{" "}
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
        </motion.span>
      </span>

      <motion.button
        className="bg-zinc-700 px-8 py-4 text-white rounded-[4px] relative overflow-hidden"
        initial={{
          translateY: "100px",
          opacity: 0,
        }}
        animate={{
          translateY: "0px",
          opacity: 1,
        }}
        transition={{
          ease: "linear",
          delay: 1.1,
          duration: 0.05,
        }}
        onClick={() => navigate("/home")}
      >
        <div className="absolute top-0 left-0 size-full animate-blink bg-[#FF3131] z-0" />
        <span className="z-20 relative">FIGHT</span>
      </motion.button>

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
