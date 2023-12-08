import React from 'react';
import ReactDOM from 'react-dom';
import { run_bf } from '../pkg'
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Stack from '@mui/material/Stack';
import Container from '@mui/material/Container';
import { styled } from '@mui/material/styles';
import Paper from '@mui/material/Paper';

const Item = styled(Paper)(({ theme }) => ({
    backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: 'center',
    color: theme.palette.text.secondary,
}));

const root = document.getElementById('root');

let predef = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."

const App = () => {
    const [prog, setProg] = React.useState(predef);
    const [input, setInput] = React.useState('');
    const [output, setOutput] = React.useState(run_bf(predef, ''));
    return (
        <Container component="main" maxWidth="xs">
            <Box sx={{ width: '100%' }}>
                <Stack spacing={2}>
                    <Item><Typography variant="h2">
                        🫠
                    </Typography></Item>
                    <Item><TextField
                        fullWidth
                        multiline
                        id="prog"
                        label="bf program"
                        name="prog"
                        variant="outlined"
                        defaultValue={prog}
                        onChange={(e) =>
                            setProg(e.target.value)
                        }
                        autoFocus /></Item>
                    <Item><TextField
                        fullWidth
                        id="input"
                        label="input buffer"
                        name="input"
                        variant="outlined"
                        defaultValue={input}
                        onChange={(e) =>
                            setInput(e.target.value)
                        }
                        autoFocus /></Item>
                    <Item><Button
                        type="submit"
                        fullWidth
                        variant="contained"
                        onClick={() => setOutput(run_bf(prog, input))}
                    >
                        Run
                    </Button></Item>
                    <Item><Typography>
                        Output: {output}
                    </Typography></Item>
                </Stack>
            </Box>
        </Container>
    );
}

ReactDOM.render(<App />, root);
