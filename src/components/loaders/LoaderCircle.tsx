function HelloWorld() {
  return (
    <>
      <span className="loader block my-10"></span>
      <style jsx>{`
        .loader {
          width: 64px;
          height: 64px;
          position: relative;
          animation: rotate 1.5s ease-in infinite alternate;
        }

        .loader::before {
          content: "";
          position: absolute;
          left: 0;
          bottom: 0;
          color: #3074f5;
          background: currentColor;
          width: 64px;
          height: 32px;
          border-radius: 0 0 50px 50px;
        }

        .loader::after {
          content: "";
          position: absolute;
          left: 50%;
          top: 10%;
          background: #031941;
          width: 8px;
          height: 64px;
          animation: rotate 1.2s linear infinite alternate-reverse;
        }

        @keyframes rotate {
          100% {
            transform: rotate(360deg);
          }
        }
      `}</style>
    </>
  );
}

export default HelloWorld;
