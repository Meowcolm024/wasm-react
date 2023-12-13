import { run_bf } from '../pkg'

import * as React from 'react';
import { createRoot } from 'react-dom/client';

import Box from "@mui/material/Box";
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';
import Paper from '@mui/material/Paper';
import Stack from '@mui/material/Stack';
import TextField from '@mui/material/TextField';
import Typography from "@mui/material/Typography";
import { styled } from '@mui/material/styles';
import { Accordion, AccordionDetails, AccordionSummary } from '@mui/material';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';

const Item = styled(Paper)(({ theme }) => ({
    backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: 'center',
    color: theme.palette.text.secondary
}));

const domNode = document.getElementById('root');
const root = createRoot(domNode);

const predef = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
const defconfig = { cells: 128, fuel: 4096 };
const MAX_CELLS = 32767;
const MAX_STEPS = Math.pow(10, 10);

function setRangeK<R>(
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
    min: number,
    max: number
): (k: (_: number) => R) => R {
    if (Number(e.target.value) < min) {
        e.target.value = min.toString();
    } else if (Number(e.target.value) > max) {
        e.target.value = max.toString();
    }
    return (k) => k(Number(e.target.value));
}

const App = () => {
    const [prog, setProg] = React.useState(predef);
    const [input, setInput] = React.useState('');
    const [config, setConfig] = React.useState(defconfig)
    const [output, setOutput] = React.useState(run_bf(predef, '', defconfig.cells, defconfig.fuel));
    return (
        <Container component="main" maxWidth="xs">
            <Box sx={{ width: '100%' }}>
                <Stack spacing={2}>
                    <Item>
                        <Typography variant="h2">
                            😆
                        </Typography>
                    </Item>

                    <Item>
                        <Box sx={{ m: 1.5 }}><TextField
                            fullWidth
                            multiline
                            id="prog"
                            label="bf program"
                            variant="outlined"
                            inputProps={{ style: { fontFamily: "monospace" } }}
                            defaultValue={prog}
                            spellCheck={false}
                            onChange={(e) => setProg(e.target.value)}
                            autoFocus /></Box>
                        <Box sx={{ m: 1.5 }}><TextField
                            fullWidth
                            id="input"
                            label="input buffer"
                            name="input"
                            variant="outlined"
                            inputProps={{ style: { fontFamily: "monospace" } }}
                            defaultValue={input}
                            spellCheck={false}
                            onChange={(e) => setInput(e.target.value)}
                        /></Box>
                        <Box sx={{ m: 1.5 }}><Stack direction="row" spacing={1}>
                            <TextField
                                fullWidth
                                id="cells"
                                label="cell size"
                                variant="outlined"
                                defaultValue={defconfig.cells}
                                onChange={(e) => setRangeK(e, 1, MAX_CELLS)((c) => setConfig({ cells: c, fuel: config.fuel }))}
                                type='number'
                            />
                            <TextField
                                fullWidth
                                id="fuel"
                                label="max steps"
                                variant="outlined"
                                defaultValue={defconfig.fuel}
                                onChange={(e) => setRangeK(e, 1, MAX_STEPS)((f) => setConfig({ cells: config.cells, fuel: f }))}
                                type='number'
                            />
                        </Stack></Box>
                        <Box sx={{ m: 1.5 }}><Button
                            type="submit"
                            fullWidth
                            variant="contained"
                            onClick={() => setOutput(run_bf(prog, input, config.cells, config.fuel))}
                        >
                            Run
                        </Button></Box>
                    </Item>

                    <Accordion defaultExpanded>
                        <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                            <Typography variant="h5">
                                Output
                            </Typography>
                        </AccordionSummary>
                        <AccordionDetails>
                            <Typography
                                variant="body1"
                                textAlign='center'
                                style={{ wordWrap: "break-word" }}
                                color="text.secondary"
                                sx={{ mb: 1.5 }}
                            >
                                {output}
                            </Typography>
                        </AccordionDetails>
                    </Accordion>
                </Stack>
            </Box>
        </Container>
    );
}

root.render(<App />);
