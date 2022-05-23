<script>
    import Base from 'svelte-chartjs/src/Base.svelte';
  let dataLine = {};
  
  const fetch_data = async () => {
    const raw = await fetch(
      "http://127.0.0.1/edsa-excel/src/controller/get_data.php"
    );
    const data = await raw.json();
    
    //divide array
    let array_data = [];
    let array_data_2 = [];
    let array_data_3 = [];

    for (let i = 0; i < 50 ; i++) {
        array_data.push(data[i]);
    }
    console.log(data);
    const giorni = array_data.map(el => {
        return el.ora_inizio + " - " + el.ora_fine;
        
    });
    const pezzi_da_produrre = array_data.map(el => {
        //return Math.random() * 1000+500;

        console.log( +el.pezzi_da_produrre)
        return +el.pezzi_da_produrre;
    });
    const pezzi_prodotti = array_data.map(el => {

        return +el.pezzi_prodotti;
    })
    const pezzi_di_scarto = array_data.map(el =>{
        //return Math.random() * 100;
        return 0;
    })
    console.log(pezzi_prodotti)
    dataLine = {
        
      labels: [...giorni],
      datasets: [
        {
          label: "Pezzi da produrre",
          fill: true,
          type:"line",
          lineTension: 0.3,
          backgroundColor: "rgba(225, 204,230, .3)",
          borderColor: "rgb(205, 130, 158)",
          borderCapStyle: "butt",
          borderDash: [],
          borderDashOffset: 0.0,
          borderJoinStyle: "miter",
          pointBorderColor: "rgb(205, 130,1 58)",
          pointBackgroundColor: "rgb(255, 255, 255)",
          pointBorderWidth: 10,
          pointHoverRadius: 5,
          pointHoverBackgroundColor: "rgb(0, 0, 0)",
          pointHoverBorderColor: "rgba(220, 220, 220,1)",
          pointHoverBorderWidth: 2,
          pointRadius: 1,
          pointHitRadius: 10,
          data: [...pezzi_da_produrre],
        },
        {
          label: "Pezzi prodotti",
          fill: true,
          type:"bar",
          lineTension: 0.3,
          borderCapStyle: "butt",
          color:"#623CEA",
          backgroundColor:"blue",
          borderColor:"#623CEA",

          borderDash: [],
          borderDashOffset: 0.0,
          borderJoinStyle: "miter",
          pointBorderWidth: 10,
          pointHoverRadius: 5,
          pointHoverBorderWidth: 2,
          pointRadius: 1,
          pointHitRadius: 10,
          data: [...pezzi_prodotti],
        },
        {
          label: "Pezzi di scarto",
          fill: true,
          type:"bar",
          lineTension: 0.3,
          borderCapStyle: "butt",
          color:"#623CEA",
          backgroundColor:"red",
          borderColor:"#623CEA",
          borderDash: [],
          borderDashOffset: 0.0,
          borderJoinStyle: "miter",
          pointBorderWidth: 10,
          pointHoverRadius: 5,
          pointHoverBorderWidth: 2,
          pointRadius: 1,
          pointHitRadius: 10,
          data: [...pezzi_di_scarto],
        },
      ],
    };
  };
  fetch_data();
</script>

<div width="500" height="500">
  <Base data={dataLine} />
  caio mdono
</div>
