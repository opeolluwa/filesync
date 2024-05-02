function Loader() {
  return (
    <>
      <span className="loader block my-10"></span>
      <style jsx>{`
        .loader,
        .loader:before {
          display: inline-block;
          border: 20px double transparent;
          border-top-color: #7EA8F9;
          border-radius: 50%;
          box-sizing: border-box;
        }

        .loader {
          padding: 8px;
          animation: wifiLoading 1s ease-in infinite;
        }

        .loader:before {
          content: "";
          width: 0;
          height: 0;
        }

        @keyframes wifiLoading {
          0% {
            border-style: none;
          }

          100% {
            border-style: double;
          }
        }
      `}</style>
    </>
  );
}

export default Loader;
