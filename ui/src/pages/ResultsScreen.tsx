import {useEffect, useRef, useState} from "react";
import {useNavigate, useParams} from "react-router-dom";
import {PieChart} from "@mui/x-charts/PieChart";
import {Box, Button, Typography} from "@mui/material";
import {io, Socket} from "socket.io-client";

type Option = {
  id: number, poll_id: number, value: string
}
type PollData = {
  id: number,
  end_date: null | string,
  start_date: string,
  title: string,
  options: Option,
}
type Result = {
  id: number,
  count: number,
  value: string,
}

const ResultsScreen = () => {
  const {id} = useParams();
  const [data, setData] = useState<null | PollData>(null);
  const [results, setResults] = useState<Result[] | null>(null);
  const socketRef = useRef<Socket | undefined>();
  const navigate = useNavigate();
  const onGoing = data?.end_date == null;

  useEffect(() => {
    Promise.all([
      fetch(`/api/polls/${id}`)
        .then((res) => res.json()),
      fetch(`/api/polls/${id}/results`)
        .then((res) => res.json())
    ]).then(([data, results]) => {
      setData(data);
      setResults(results);
    })
  }, []);

  useEffect(() => {
    if (!onGoing) {
      return
    }

    const socket = io(window.location.origin);
    socketRef.current = socket;

    socket.emit("subscribe", id);
    socket.on("vote", (...x) => setResults(x));
    socket.on("end", setPollAsEnded)


    return () => {
      socket.disconnect();
      socketRef.current = undefined;
    }
  }, [data, id]);

  const setPollAsEnded = () => {
    setData(data => {
      if (data == null) return data

      return {...data, end_date: new Date().toISOString()}
    })
  }

  const handleEndPoll = () => {
    fetch(`/api/polls/${id}/end`, {
      method: "POST",
    }).then(setPollAsEnded)

  }

  const handleCreateNew = () => navigate("/");

  if (data == null || results == null) {
    return (
      <Box
        height="100%"
        width="100%"
        display="flex"
        alignItems="center"
        justifyContent="center"
        flexDirection="column"
        gap="4"
        p={12}
      >
        <Typography variant="h1">Cargando...</Typography>
      </Box>
    )
  }

  return (
    <Box
      height="100%"
      width="100%"
      display="flex"
      flexDirection="column"
      alignItems="center"
      justifyContent="center"
      gap={4}
      sx={{p: 12}}
    >
      <Typography variant="h2">{data.title}</Typography>

      {results.every(x => x.count === 0) ? (
        onGoing ? (
          <Typography variant="h4">Todavía no hay votos</Typography>
        ) : (
          <Typography variant="h4">La encuesta ha terminado. No hay votos que mostrar.</Typography>
        )
      ) : (
        <PieChart
          series={[{
            data: results.map(x => {
              return {
                id: x.id,
                value: x.count,
                label: x.value,
              }
            })
          }]}
          width={400}
          height={200}
        />
      )}

      {data.end_date == null ? (
        <Button variant="contained" onClick={handleEndPoll}>Terminar Encuesta</Button>
      ) : (
        <Button variant="outlined" onClick={handleCreateNew}>Crea una nueva encuesta</Button>
      )}
    </Box>
  )
}

export default ResultsScreen;
