import AppBase from "../../base/AppBase";
const Home = () => {
  return (
    <AppBase>
      <div>
        <h1>Welcome to WCE (World Coding Entertainment)</h1>
        <p>
          WCE stands for "World Coding Entertainment", inspired by WWE (World Wrestling Entertainment). 
          Unlike WWE, the stakes are high, and the injuries are real! Compete with live programmers and dominate the ring with intellect and skill.
        </p>
        <p>
          The frontend is built with React using Vite, and the backend uses a microservice architecture with Rust and Docker.
        </p>
      </div>
    </AppBase>
  );
};

export default Home;
