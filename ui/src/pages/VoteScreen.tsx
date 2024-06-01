import {useEffect, useState} from "react";
import {useNavigate, useParams} from "react-router-dom";
import {
  Box,
  Card,
  CardActionArea,
  CardContent,
  Stack,
  Typography
} from "@mui/material";

type Option = {
  id: number, poll_id: number, value: string
}
type PollData = {
  id: number,
  end_date: null | string,
  start_date: string,
  title: string,
  options: Option[],
}

const VoteScreen = () => {
  const {id} = useParams();
  const navigate = useNavigate()
  const [data, setData] = useState<null | PollData>(null);

  useEffect(() => {
    fetch(`/api/polls/${id}`)
      .then((res) => res.json())
      .then((data) => {
        if (data.end_date) {
          return navigate(`/${id}/results`);
        }
        setData(data);
      })
  }, []);

  const handleVote = (option: Option) => {
    const pollId = option.poll_id;
    const answer_id = option.id;

    fetch(`/api/polls/${pollId}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ answer_id }),
    }).then(() => navigate(`/${id}/results`));
  }

  if (data == null) {
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
      sx={{px: { xs: 4, md: 12 }, py: 12}}
    >
      <Typography variant="h2">{data.title}</Typography>

      <Stack
        gap={1}
        sx={{width: "100%", maxWidth: 800}}
      >
        {data.options.map((option) => {
          return (
            <Card
              variant="outlined"
              key={option.id}
            >
              <CardActionArea onClick={handleVote.bind(null, option)}>
                <CardContent>
                  <Typography
                    variant="body2"
                    sx={{whiteSpace: 'normal', wordWrap: 'break-word'}}
                  >
                    {option.value}
                  </Typography>
                </CardContent>
              </CardActionArea>
            </Card>
          )
        })}
      </Stack>
    </Box>
  )
}

export default VoteScreen;
