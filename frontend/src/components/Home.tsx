import { useState } from "react";
import Fight from "./Fight";

const Home = () => {
  const [isFighting, setIsFighting] = useState(true);

  return isFighting ? (
    <Fight />
  ) : (
    <div>
      Home
    </div>
  )

}

export default Home;
