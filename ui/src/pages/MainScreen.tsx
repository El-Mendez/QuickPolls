import {Link} from "react-router-dom";
import {Box, Button, Typography} from "@mui/material";

const MainScreen = () => {
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
      <Typography variant="h1">Crea una encuesta</Typography>
      <Link to="/crear">
        <Button>
          <Typography variant="h2">WooooW</Typography>
        </Button>
      </Link>
    </Box>
  )
}

export default MainScreen;
