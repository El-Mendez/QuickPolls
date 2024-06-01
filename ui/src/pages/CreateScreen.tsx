import {useState} from "react";
import {Box, Button, Card, CardContent, Grid, IconButton, Stack, TextField, Typography} from "@mui/material";
import {Delete} from "@mui/icons-material";
import {useNavigate} from "react-router-dom";

const CreateScreen = () => {
  const [options, setOptions] = useState<string[]>([]);
  const [title, setTitle] = useState<string>("");
  const [currentOption, setCurrentOption] = useState<string>("");
  const navigate = useNavigate();

  const addOption = () => {
    if (currentOption !== "")
      setOptions([...options, currentOption]);
    setCurrentOption("");
  }

  const removeOption = (index: number) => {
    setOptions([...options.slice(0, index), ...options.slice(index + 1, options.length)]);
  }

  const onSubmit = async () => {
    try {
      const data = await fetch("http://localhost:3000/api/polls", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({title, options}),
      });
      const json = await data.json();

      navigate(`/${json.id}/results`);
    } catch (e) {
      console.log(e)
    }
  };

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
      <Typography variant="h2">Crea una encuesta</Typography>

      <TextField
        id="standard-basic"
        label="Pregunta"
        variant="standard"
        value={title}
        onChange={x => setTitle(x.target.value)}
        sx={{width: "30%"}}
      />

      <Stack
        gap={1}
        sx={{width: "70%"}}
      >
        {options.map((option, index) => {
          return (
            <Card variant="outlined">
              <CardContent>
                <Grid
                  container
                  direction="row"
                  wrap="nowrap"
                  width="100%"
                  alignItems="center"
                >
                  <Grid item xs={11}>
                    <Typography
                      variant="body2"
                      sx={{whiteSpace: 'normal', wordWrap: 'break-word'}}
                    >
                      {option}
                    </Typography>
                  </Grid>
                  <Grid item xs={1}>
                    <IconButton onClick={removeOption.bind(null, index)}>
                      <Delete/>
                    </IconButton>
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          )
        })}
      </Stack>

      <Stack
        direction="row"
        gap={2}
        justifyContent="center"
        alignItems="center"
        sx={{width: "70%"}}
      >
        <TextField
          id="standard-basic"
          label="Respuesta"
          variant="standard"
          value={currentOption}
          onChange={x => setCurrentOption(x.target.value)}
          sx={{width: "70%"}}
        />
        <Button
          variant="text"
          disabled={currentOption === ""}
          onClick={addOption}
        >
          Añadir
        </Button>
      </Stack>

      <Button
        variant="contained"
        disabled={title === "" || options.length === 0}
        onClick={onSubmit}
      >
        Crear
      </Button>
    </Box>
  )
}

export default CreateScreen;
