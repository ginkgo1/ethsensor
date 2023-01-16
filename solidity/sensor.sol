pragma solidity ^0.8.7;

struct Measurement {
    uint timestamp;
    int32 temperature;
}


contract sensors {

    address raspberry;
    Measurement[] measurements;

    /**
     * Constructor del contrato. Se anota la dirección que crea
     * el contrato porque será el único autorizado a añadir valores.
     */
    constructor() {
        raspberry = msg.sender;
    }
    
    function save(int32 temp, uint timestamp) public {
        require(msg.sender == raspberry);
        Measurement memory m;
        m.temperature = temp;
        m.timestamp = timestamp;
        measurements.push(m);
    }

    function save_many(int32[] calldata temps, uint[] calldata timestamps) public {
        require(msg.sender == raspberry);
        for (uint i = 0; i < temps.length; ++i) {
            Measurement memory m;
            m.temperature = temps[i];
            m.timestamp = timestamps[i];
            measurements.push(m);
        }
    }

    function read(uint numval) public view returns (int32[] memory, uint[] memory) {
        if (numval > measurements.length) {
            numval = measurements.length;
        }
        int32[] memory temps = new int32[](numval);
        uint[] memory times = new uint[](numval);

        for(uint i = 0; i < numval; i++) {
            temps[i] = measurements[measurements.length - numval + i].temperature;
            times[i] = measurements[measurements.length - numval + i].timestamp;
        }
        return (temps, times);
    }

    function clear() public {
        require(msg.sender == raspberry);
        delete measurements;
    }
}
