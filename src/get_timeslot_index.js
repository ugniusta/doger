const table = document.querySelector('table')
table.addEventListener('click', (event) => {
        const cell = event.target.closest('td[kennel_id][column]');

        if (cell) {
            const column = cell.getAttribute('column');
            const kennel_id = cell.getAttribute('kennel_id');

            const dayRow = cell.closest('tr[day]');
            const day = dayRow ? dayRow.getAttribute('day') : null;

            const monthTbody = cell.closest('tbody[year-month]');
            const year_month = monthTbody ? monthTbody.getAttribute('year-month') : null;

            dioxus.send({
                date: `${year_month}-${day}`,
                kennel_id: parseInt(kennel_id, 10),
                column: parseInt(column, 10)
            })
        }
    }
    ,
    {once: true}
)